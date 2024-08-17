// built-in features for the runtime.
const deno = Object.assign({}, Deno);
delete Deno;

const { ops } = deno.core;

globalThis.setTimeout = (cb, delay) => {
  ops.set_timeout(delay).then(cb);
};
