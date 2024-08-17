import { doThing } from "./lib.ts";

if (doThing()) {
  const start = Date.now();

  setTimeout(() => {
    const end = Date.now();
    console.log(`Execution time: ${end - start} milliseconds`);
  }, 100);
} else {
  throw new Error("failed to load lib.");
}

