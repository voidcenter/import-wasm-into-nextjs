/** @type {import('next').NextConfig} */


module.exports = {
    cleanDistDir: false,
    webpack: (config, { isServer }) => {
      // Add WebAssembly support
      if (!isServer) {
        config.experiments.asyncWebAssembly = true;
        config.module.rules.push({
          test: /\.wasm$/,
          type: 'webassembly/async',
        });
      } 
  
      // config.plugins.push(
      //   new (class {
      //     apply(compiler) {
      //       compiler.hooks.afterEmit.tapPromise(
      //         'SymlinkWebpackPlugin',
      //         async (compiler) => {
      //           if (isServer) {
      //             const from = join(compiler.options.output.path, '../static');
      //             const to = join(compiler.options.output.path, 'static');
      
      //             try {
      //               await access(from);
      //               console.log(`${from} already exists`);
      //               return;
      //             } catch (error) {
      //               if (error.code === 'ENOENT') {
      //                 // No link exists
      //               } else {
      //                 throw error;
      //               }
      //             }
      
      //             await symlink(to, from, 'junction');
      //             console.log(`created symlink ${from} -> ${to}`);
      //           }
      //         },
      //       );
      //     }
      //   })(),
      // );


      return config;
    },
};


