// Snowpack Configuration File
// See all supported options: https://www.snowpack.dev/reference/configuration

/** @type {import("snowpack").SnowpackUserConfig } */
module.exports = {
  mount: {
    "public": {url: "/", static: true},
    "web": {url: "/web"},
  },
  plugins: [
    [
      "snowpack-plugin-wasm-pack",
      {
        projectPath: './'
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
