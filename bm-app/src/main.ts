import {createApp} from "vue";
import App from "./App.vue";
import init, {BrownianMotion} from "brownian-motion";


createApp(App).mount("#app");
async function run() {
    await init();
    let bm = BrownianMotion.new(10, 1);
    let t = bm.times(0.1);
    let x = bm.simulate(0.1);
    console.log(t);
    console.log(x);
}
run();

// onMounted(()=> {
//     init().then(() => {
//         run();
//     })
// })
