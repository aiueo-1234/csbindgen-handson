using CsbindgenHandsOn;

var c = new BasicFunctionCall();
c.CallRustAdd(1,1);
c.CallMyMathAdd(2,2);
c.CallRustPow(2,3);

using var t = new TestGroupedNativeMethods();
t.PushAndPop([1,2,3,4,5]);
