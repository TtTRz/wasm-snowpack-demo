// Snowpack Configuration File
// See all supported options: https://www.snowpack.dev/reference/configuration

/** @type {import("snowpack").SnowpackUserConfig } */
module.exports = {
  mount: {
    "public": {url: "/", static: true},
    "src": {url: "/src"},
  },
  plugins: [
    [
      "snowpack-plugin-wasm-pack",
      {
        projectPath: './rust-crate'
      }
    ]
  ],
  packageOptions: {
    /* ... */
  },
  devOptions: {
    port: 8080,
    open: "chrome",
    /* ... */
  },
  buildOptions: {
    /* ... */
  },
};
