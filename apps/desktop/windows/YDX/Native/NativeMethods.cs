using System.Runtime.InteropServices;

namespace YDX.Native
{
    internal static class NativeMethods
    {
        private const string DllName = @"Native\ydx_windows.dll";

        [DllImport(DllName, CallingConvention = CallingConvention.Cdecl)]
        public static extern ulong ydx_windows_performance_test();
    }
}