package com.talon.studytracker

import android.app.AlarmManager
import android.content.Context
import android.content.pm.ActivityInfo
import android.os.Build
import android.os.Bundle
import android.view.WindowManager
import android.webkit.JavascriptInterface
import android.webkit.WebView
import androidx.activity.enableEdgeToEdge
import androidx.core.view.WindowCompat
import androidx.core.view.WindowInsetsCompat
import androidx.core.view.WindowInsetsControllerCompat

class MainActivity : TauriActivity() {
  override fun onCreate(savedInstanceState: Bundle?) {
    enableEdgeToEdge()
    super.onCreate(savedInstanceState)
  }

  // window.STNative, read by the NATIVE section of index.html (B5). Only the
  // app's own bundled pages load in this WebView, so the bridge is not
  // reachable from the web.
  override fun onWebViewCreate(webView: WebView) {
    webView.addJavascriptInterface(Bridge(), "STNative")
  }

  inner class Bridge {
    // A1/A2: false means the plugin has quietly fallen back to inexact alarms.
    @JavascriptInterface
    fun exactAlarms(): Boolean {
      if (Build.VERSION.SDK_INT < Build.VERSION_CODES.S) return true
      val am = getSystemService(Context.ALARM_SERVICE) as AlarmManager
      return am.canScheduleExactAlarms()
    }

    // Picks which battery steps the first-run card shows (A6/B6: Samsung only
    // needs "Never sleeping apps").
    @JavascriptInterface
    fun maker(): String = Build.MANUFACTURER ?: ""

    @JavascriptInterface
    fun sdk(): Int = Build.VERSION.SDK_INT

    // The one battery state Android lets an app read. Samsung's sleep lists
    // have no API, so Settings can only ask the user to confirm those.
    @JavascriptInterface
    fun batteryExempt(): Boolean {
      val pm = getSystemService(Context.POWER_SERVICE) as android.os.PowerManager
      return pm.isIgnoringBatteryOptimizations(packageName)
    }

    // Opens the settings screen where the user grants each thing. Falls back
    // to the app's own details page, which every Android and skin has.
    @JavascriptInterface
    fun openSettings(what: String) {
      runOnUiThread {
        val pkg = android.net.Uri.parse("package:$packageName")
        val intent = when (what) {
          "notifications" -> android.content.Intent(android.provider.Settings.ACTION_APP_NOTIFICATION_SETTINGS)
            .putExtra(android.provider.Settings.EXTRA_APP_PACKAGE, packageName)
          "exactAlarms" -> if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.S)
            android.content.Intent(android.provider.Settings.ACTION_REQUEST_SCHEDULE_EXACT_ALARM, pkg) else null
          "battery" -> android.content.Intent(android.provider.Settings.ACTION_IGNORE_BATTERY_OPTIMIZATION_SETTINGS)
          else -> null
        } ?: android.content.Intent(android.provider.Settings.ACTION_APPLICATION_DETAILS_SETTINGS, pkg)
        try { startActivity(intent) } catch (e: Exception) {
          startActivity(android.content.Intent(android.provider.Settings.ACTION_APPLICATION_DETAILS_SETTINGS, pkg))
        }
      }
    }

    // A9: the WebView has no download handler and can't hand a blob URL to
    // anything outside itself, so export writes to Downloads through
    // MediaStore. Returns the name the file was stored under, only once the
    // bytes are written and the entry is published, or "" if it wasn't.
    // Before Android 10 there is no Downloads collection, so it says "" and
    // the page reports the failure. A second export the same day lands as
    // "name (1).json" because MediaStore picks the name, so the page prints
    // the stored name, never the one it asked for.
    @JavascriptInterface
    fun saveFile(name: String, mime: String, text: String): String {
      if (Build.VERSION.SDK_INT < Build.VERSION_CODES.Q) return ""
      val values = android.content.ContentValues().apply {
        put(android.provider.MediaStore.Downloads.DISPLAY_NAME, name)
        put(android.provider.MediaStore.Downloads.MIME_TYPE, mime)
        put(android.provider.MediaStore.Downloads.IS_PENDING, 1)
      }
      val uri = contentResolver.insert(android.provider.MediaStore.Downloads.EXTERNAL_CONTENT_URI, values)
        ?: return ""
      return try {
        contentResolver.openOutputStream(uri)!!.use { it.write(text.toByteArray(Charsets.UTF_8)) }
        values.clear()
        values.put(android.provider.MediaStore.Downloads.IS_PENDING, 0)
        contentResolver.update(uri, values, null, null)
        contentResolver.query(uri, arrayOf(android.provider.MediaStore.Downloads.DISPLAY_NAME),
          null, null, null)?.use { c -> if (c.moveToFirst()) c.getString(0) else null } ?: name
      } catch (e: Exception) {
        contentResolver.delete(uri, null, null)
        ""
      }
    }

    // The app-drawn AOD (PLAN-android.md A13), ported unchanged from Wellness
    // Hub. No Android version on the 2a lets a third-party app draw on the
    // real AOD, so while the focus face is up the app shows over the lock
    // screen, keeps the display awake and drops to near-minimum brightness.
    // Pressing power still turns the screen off.
    @JavascriptInterface
    fun face(on: Boolean) {
      runOnUiThread {
        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.O_MR1) setShowWhenLocked(on)
        val lp = window.attributes
        lp.screenBrightness = if (on) FACE_BRIGHTNESS else WindowManager.LayoutParams.BRIGHTNESS_OVERRIDE_NONE
        window.attributes = lp
        if (on) window.addFlags(WindowManager.LayoutParams.FLAG_KEEP_SCREEN_ON)
        else window.clearFlags(WindowManager.LayoutParams.FLAG_KEEP_SCREEN_ON)
        // The face is laid out for portrait and uses the whole screen, so it
        // holds portrait and hides the status and nav bars while it is up.
        // A swipe from an edge still brings the bars back briefly.
        requestedOrientation = if (on) ActivityInfo.SCREEN_ORIENTATION_PORTRAIT
          else ActivityInfo.SCREEN_ORIENTATION_UNSPECIFIED
        val bars = WindowCompat.getInsetsController(window, window.decorView)
        if (on) {
          bars.systemBarsBehavior = WindowInsetsControllerCompat.BEHAVIOR_SHOW_TRANSIENT_BARS_BY_SWIPE
          bars.hide(WindowInsetsCompat.Type.systemBars())
        } else bars.show(WindowInsetsCompat.Type.systemBars())
      }
    }

    // Keep-awake alone: on while a block or break counts, off when the face
    // is paused or idle so the screen may time out.
    @JavascriptInterface
    fun awake(on: Boolean) {
      runOnUiThread {
        if (on) window.addFlags(WindowManager.LayoutParams.FLAG_KEEP_SCREEN_ON)
        else window.clearFlags(WindowManager.LayoutParams.FLAG_KEEP_SCREEN_ON)
      }
    }
  }


  companion object {
    // 0.05 read as too dim on the 2a (Helth, 2026-09-23); 0.10 is what shipped.
    const val FACE_BRIGHTNESS = 0.10f
  }
}
