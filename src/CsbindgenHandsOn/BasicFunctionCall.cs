namespace CsbindgenHandsOn;

public class BasicFunctionCall
{
    public void CallRustAdd(int a, int b){
        Console.WriteLine($"rust_add({a}, {b}): {Native.NativeMethods.rust_add(a,b)}");
    }

    public void CallMyMathAdd(int a, int b){
        Console.WriteLine($"myMath_add({a}, {b}): {Native.CNativeMethodsMyMath.myMath_add(a,b)}");
    }
}

