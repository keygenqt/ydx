using System.Runtime.InteropServices;

namespace DemoRust
{
    internal static class NativeMethods
    {
        private const string DllName = @"Native\shared.dll";

        [DllImport(DllName, CallingConvention = CallingConvention.Cdecl)]
        public static extern ulong shared_performance_test();
    }
}