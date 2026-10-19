/**
 * Hourglass: a glass in a three-post frame, every grain of sand still in the
 * top bulb and the bottom one empty. The pointer tips it toward the side it
 * is on, on a spring, rocking on the rim of its base; the packed sand tips
 * with it and none falls. The slider is the furthest it leans, in degrees.
 *
 * The pattern: a continuous input, as Terrain, but one number for the whole
 * object. The lean is read from the pointer's screen x against a fixed
 * centre, so tipping never moves the thing it is tested against.
 */
const {
  Cam, circ, clamp, fit, hull, poly, proj, rad, ringAt, spring, stepS,
  disposer, mk, pointer, register,
} = HL;

const R = 30, CAP = 5, H = 120, NZ = 60, PR = 24, POST = 2.3, YAW = 20, SAND = 30;
// One bulb's radius by height above the neck: a swell from the 3-unit neck to 20, closing to 6 at the cap.
// The bottom bulb is the same, mirrored. Sampled every 2.5 units so the hull reads as a curve, not facets.
const rAt = (dz) => (dz <= 30 ? 3 + 17 * Math.sin((Math.PI / 2) * (dz / 30)) ** 0.8 : 20 - 14 * ((dz - 30) / 25) ** 2);
const BULB = Array.from({ length: 23 }, (_, i) => [i * 2.5, rAt(i * 2.5)]);
const POSTS = [0, 1, 2].map((k) => rad(YAW + 90 + 120 * k)).map((a) => [PR * Math.cos(a), PR * Math.sin(a)]);

/** World points tipped by theta degrees toward screen right (negative: left), about the base rim on that side. */
function tipper(theta) {
  const t = rad(Math.abs(theta)), c = Math.cos(t), s = Math.sin(t), sg = theta < 0 ? -1 : 1, k = Math.SQRT1_2;
  return (x, y, z) => {
    const a = sg * (x - y) * k - R, b = (x + y) * k;
    const A = sg * (a * c + z * s + R);
    return [(A + b) * k, (b - A) * k, -a * s + z * c];
  };
}

function mount({ stage, svg, read }, value) {
  const bag = disposer();
  let most = value;

  const C = Cam(45, 0.5, 1.98);
  const ends = [];
  for (const th of [-28, 0, 28]) { const T = tipper(th); for (const [x, y] of [[R, R], [-R, -R], [R, -R], [-R, R]]) for (const z of [0, H]) ends.push(T(x * 0.72, y * 0.72, z)); }
  fit(C, ends, 200, 166);
  const P0 = proj(C);

  const g = mk("g", {}, svg);
  const el = (cls) => mk("path", { class: cls }, g);
  // Paint order: base, the posts behind the glass, sand, glass, the posts in front, the cap.
  const depth = ([x, y]) => x + y;
  const back = POSTS.filter((p) => depth(p) < 0), front = POSTS.filter((p) => depth(p) >= 0);
  const base = el("sil"), baseCr = el("nf lo");
  const backEls = back.map(() => el("sil"));
  const sand = el("sil"), sandTop = el("nf lo");
  // The glass is dim and the sand carries the silhouette; the empty bottom bulb is the bright mark.
  const upper = el("nf lo"), lower = el("nf hi"), neck = el("nf lo");
  const frontEls = front.map(() => el("sil"));
  const cap = el("sil"), capCr = el("nf lo");

  const lean = spring(0);
  let drawn = NaN;

  function draw() {
    const th = lean.x;
    if (th === drawn) return;
    drawn = th;
    const T = tipper(th), Pw = (x, y, z) => P0(...T(x, y, z));
    const ring = (r, z, x = 0, y = 0, n = 32) => ringAt(Pw, circ(r, n).map((q) => ({ ...q, u: q.u + x, v: q.v + y })), z);
    const solidOf = (rs) => poly(hull(rs.flat()));
    base.setAttribute("d", solidOf([ring(R, 0), ring(R, CAP)]));
    baseCr.setAttribute("d", poly(ring(R - 2, CAP)));
    cap.setAttribute("d", solidOf([ring(R, H - CAP), ring(R, H)]));
    capCr.setAttribute("d", poly(ring(R - 2, H)));
    const post = ([x, y]) => solidOf([ring(POST, CAP, x, y, 12), ring(POST, H - CAP, x, y, 12)]);
    back.forEach((p, i) => backEls[i].setAttribute("d", post(p)));
    front.forEach((p, i) => frontEls[i].setAttribute("d", post(p)));
    upper.setAttribute("d", solidOf(BULB.map(([dz, r]) => ring(r, NZ + dz))));
    lower.setAttribute("d", solidOf(BULB.map(([dz, r]) => ring(r, NZ - dz))));
    neck.setAttribute("d", poly(ring(3, NZ, 0, 0, 16)));
    // The sand: packed into the top bulb from the neck up, one grain-width inside the glass.
    const levels = BULB.filter(([dz]) => dz < SAND).map(([dz]) => dz).concat(SAND);
    sand.setAttribute("d", solidOf(levels.map((dz) => ring(Math.max(1.6, rAt(dz) - 1.4), NZ + dz))));
    sandTop.setAttribute("d", poly(ring(rAt(SAND) - 3.2, NZ + SAND)));
  }

  const B = register(stage, (dt) => { const m = stepS(lean, dt); draw(); return m; });
  bag.add(B.unregister);

  // The pointer's side of a fixed centre line sets the lean; the read-out counts the sand that has fallen: none.
  const mid = P0(0, 0, NZ)[0];
  function aim(p) {
    if (p) {
      lean.t = clamp((p[0] - mid) / 110, -1, 1) * most;
      read.textContent = `${Math.round(Math.abs(lean.t))}° · 0:00`;
    } else { lean.t = 0; read.textContent = "rest"; }
    B.wake();
  }

  bag.add(pointer(stage, { move: aim, leave: () => aim(null) }));
  bag.add(() => svg.replaceChildren());

  return {
    set: (v) => { most = v; },
    destroy: bag.dispose,
  };
}

hairline({
  name: "hourglass",
  means: "An hourglass with every grain still on top: the pointer tips it toward its side, and none of the sand falls.",
  rules: [1, 3, 5, 6],
  range: [12, 20, 28],
  mount,
});
