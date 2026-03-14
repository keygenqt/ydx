using System.Runtime.InteropServices;
using System.Text;

namespace YDX.Native
{
    internal static class NativeMethods
    {
        private const string DllName = @"Native\ydx_windows.dll";

        // Init http client
        [DllImport(DllName, CallingConvention = CallingConvention.Cdecl)]
        private static extern void ydx_windows_request_init(byte[] base_url, byte[] uuid);

        // Free function
        [DllImport(DllName, CallingConvention = CallingConvention.Cdecl)]
        internal static extern void ydx_windows_free_string(IntPtr ptr, ulong len, ulong cap);

        // Signin user
        [DllImport(DllName, CallingConvention = CallingConvention.Cdecl)]
        private static extern VecUint8 ydx_windows_request_signin(byte[] email, byte[] password);

        // Signup user
        [DllImport(DllName, CallingConvention = CallingConvention.Cdecl)]
        private static extern VecUint8 ydx_windows_request_signup(byte[] email, byte[] password);

        // Check auth
        [DllImport(DllName, CallingConvention = CallingConvention.Cdecl)]
        private static extern VecUint8 ydx_windows_request_signcheck(byte[] token);

        // Get user
        [DllImport(DllName, CallingConvention = CallingConvention.Cdecl)]
        private static extern VecUint8 ydx_windows_request_get_user(byte[] id, byte[] token);

        // Signout user
        [DllImport(DllName, CallingConvention = CallingConvention.Cdecl)]
        private static extern VecUint8 ydx_windows_request_signout(byte[] token);

        // ========== ПУБЛИЧНЫЕ МЕТОДЫ (работают со string) ==========

        public static void Init(string baseUrl, string uuid)
        {
            byte[] baseUrlBytes = Encoding.UTF8.GetBytes(baseUrl + '\0');
            byte[] uuidBytes = Encoding.UTF8.GetBytes(uuid + '\0');
            ydx_windows_request_init(baseUrlBytes, uuidBytes);
        }

        public static string SignIn(string email, string password)
        {
            byte[] emailBytes = Encoding.UTF8.GetBytes(email + '\0');
            byte[] passwordBytes = Encoding.UTF8.GetBytes(password + '\0');
            VecUint8 result = ydx_windows_request_signin(emailBytes, passwordBytes);
            string str = result.ToUtf8String();
            result.Free();
            return str;
        }

        public static string SignUp(string email, string password)
        {
            byte[] emailBytes = Encoding.UTF8.GetBytes(email + '\0');
            byte[] passwordBytes = Encoding.UTF8.GetBytes(password + '\0');
            VecUint8 result = ydx_windows_request_signup(emailBytes, passwordBytes);
            string str = result.ToUtf8String();
            result.Free();
            return str;
        }

        public static string SignCheck(string token)
        {
            byte[] tokenBytes = Encoding.UTF8.GetBytes(token + '\0');
            VecUint8 result = ydx_windows_request_signcheck(tokenBytes);
            string str = result.ToUtf8String();
            result.Free();
            return str;
        }

        public static string GetUser(string id, string token)
        {
            byte[] idBytes = Encoding.UTF8.GetBytes(id + '\0');
            byte[] tokenBytes = Encoding.UTF8.GetBytes(token + '\0');
            VecUint8 result = ydx_windows_request_get_user(idBytes, tokenBytes);
            string str = result.ToUtf8String();
            result.Free();
            return str;
        }

        public static string SignOut(string token)
        {
            byte[] tokenBytes = Encoding.UTF8.GetBytes(token + '\0');
            VecUint8 result = ydx_windows_request_signout(tokenBytes);
            string str = result.ToUtf8String();
            result.Free();
            return str;
        }
    }

    [StructLayout(LayoutKind.Sequential)]
    public struct VecUint8
    {
        public IntPtr ptr;
        public ulong len;
        public ulong cap;

        public string ToUtf8String()
        {
            if (ptr == IntPtr.Zero || len == 0)
                return string.Empty;

            byte[] bytes = new byte[len];
            Marshal.Copy(ptr, bytes, 0, (int)len);
            return Encoding.UTF8.GetString(bytes);
        }

        public void Free()
        {
            if (ptr != IntPtr.Zero)
            {
                NativeMethods.ydx_windows_free_string(ptr, len, cap);
                ptr = IntPtr.Zero;
                len = 0;
                cap = 0;
            }
        }
    }
}