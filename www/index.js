import { WebBefunge } from "../pkg/befunge_rust";

export async function run_befunge(program, input) {
    try {
        const interpreter = new WebBefunge(program);
        interpreter.set_input(input);
        return interpreter.run();
    } catch (e) {
        console.error('Befunge execution error:', e);
        throw e;
    }
}