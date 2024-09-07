using CsbindgenHandsOn.Native;

namespace CsbindgenHandsOn
{
    public sealed unsafe class TestGroupedNativeMethods : IDisposable
    {
        private bool _disposed;
        private readonly MyStack* _stack;

        public TestGroupedNativeMethods()
        {
            _stack = CNativeMethodsMyStack.myStack_create(5);
        }

        public void PushAndPop(ReadOnlySpan<int> numbers)
        {
            for (int i = 0; i < numbers.Length && i < 5; i++)
            {
                _stack->Push(numbers[i]);
                Console.WriteLine($"pushed {numbers[i]}");
            }
            for (int i = 0;i < numbers.Length && i != 5; i++)
            {
                Console.WriteLine($"popped {_stack->Pop()}");
            }
        }

        public void Dispose()
        {
            if (_disposed) return;
            _stack->Delete();
            _disposed = true;
        }
    }
}
