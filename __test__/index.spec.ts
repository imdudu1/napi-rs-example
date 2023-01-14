import test from "ava";

import { asyncPlus100, minus100, plus100 } from "../index";

test("sync plus100 function from native code", (t) => {
  const fixture = 42;
  t.is(plus100(fixture), fixture + 100);
});

test("sync minus100 function from native code", (t) => {
  const fixture = 42;
  t.is(minus100(fixture), fixture - 100);
});

test("asyncPlus100 function from native code", async (t) => {
  const fixture = new Promise<number>((resolve) => {
    setTimeout(() => resolve(20), 50);
  });

  const actual = await asyncPlus100(fixture);

  t.is(actual, 120);
});
