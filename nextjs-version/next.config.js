/** @type {import('next').NextConfig} */

module.exports = {
    webpack: (config, { isServer }) => {
      // Add WebAssembly support
      if (!isServer) {
        config.experiments.asyncWebAssembly = true;
        config.module.rules.push({
          test: /\.wasm$/,
          type: 'webassembly/async',
        });
      }
  
      return config;
    },
};


