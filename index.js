import init, { run } from './pkg/vrc.js';
(async () => {
    await init();
    run();
})();
