import test from "ava";

import { asyncPlus100, minus100, Person, plus100 } from "../index";

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

test("reject asyncPlus100 function from native code", async (t) => {
  const fixture = new Promise<number>((_, reject) => {
    setTimeout(() => reject(new Error("Oops!")), 50);
  });

  const exepctionAction = async () => asyncPlus100(fixture);

  await t.throwsAsync(exepctionAction);
});

test("withNameAndAge", async (t) => {
  const fixture = Person.withNameAndAge("yohan", 20);

  const actualName = fixture.name;
  const actualAge = fixture.age;

  t.is(actualName, "yohan");
  t.is(actualAge, 20);
});

test("increaseAge", async (t) => {
  const fixture = new Person();

  fixture.increaseAge();

  t.is(fixture.age, 1);
});

test("changeName", async (t) => {
  const fixture = new Person();
  const expectName = "New yohan";

  fixture.changeName(expectName);

  t.is(fixture.name, expectName);
});
