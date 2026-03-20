using System;
using System.Runtime.InteropServices;
using System.Text;

namespace YDX.Native
{
    internal static class NativeMethods
    {
        private const string DllName = @"Native\ydx_windows.dll";

        [StructLayout(LayoutKind.Sequential)]
        private struct RustString
        {
            public IntPtr ptr;
            public ulong len;
            public ulong capacity;
        }

        [DllImport(DllName, CallingConvention = CallingConvention.Cdecl)]
        internal static extern void ydx_windows_free_string(IntPtr ptr, ulong len, ulong cap);

        [DllImport(DllName, CallingConvention = CallingConvention.Cdecl)]
        private static extern void ydx_windows_request_init(ref RustString base_url, ref RustString uuid);

        [DllImport(DllName, CallingConvention = CallingConvention.Cdecl)]
        private static extern VecUint8 ydx_windows_request_signin(ref RustString email, ref RustString password);

        [DllImport(DllName, CallingConvention = CallingConvention.Cdecl)]
        private static extern VecUint8 ydx_windows_request_signup(ref RustString email, ref RustString password);

        [DllImport(DllName, CallingConvention = CallingConvention.Cdecl)]
        private static extern VecUint8 ydx_windows_request_signcheck(ref RustString token);

        [DllImport(DllName, CallingConvention = CallingConvention.Cdecl)]
        private static extern VecUint8 ydx_windows_request_get_user(ref RustString id, ref RustString token);

        [DllImport(DllName, CallingConvention = CallingConvention.Cdecl)]
        private static extern VecUint8 ydx_windows_request_signout(ref RustString token);

        private static RustString _baseUrlStr;
        private static RustString _uuidStr;
        private static bool _initialized = false;

        private static string PrepareForFfi(string input)
        {
            if (string.IsNullOrEmpty(input))
                return input ?? string.Empty;

            var result = new StringBuilder(input.Length);
            foreach (char c in input)
            {
                if (!char.IsControl(c) && c <= 127)
                    result.Append(c);
            }
            return result.ToString().Trim();
        }

        public static void HttpClientInit(string baseUrl, string uuid)
        {
            if (_initialized) return;

            try
            {
                string cleanBaseUrl = PrepareForFfi(baseUrl);
                string cleanUuid = PrepareForFfi(uuid);

                byte[] baseUrlBytes = Encoding.UTF8.GetBytes(cleanBaseUrl + '\0');
                byte[] uuidBytes = Encoding.UTF8.GetBytes(cleanUuid + '\0');

                IntPtr baseUrlPtr = Marshal.AllocHGlobal(baseUrlBytes.Length);
                IntPtr uuidPtr = Marshal.AllocHGlobal(uuidBytes.Length);

                try
                {
                    Marshal.Copy(baseUrlBytes, 0, baseUrlPtr, baseUrlBytes.Length);
                    Marshal.Copy(uuidBytes, 0, uuidPtr, uuidBytes.Length);

                    _baseUrlStr = new RustString
                    {
                        ptr = baseUrlPtr,
                        len = (ulong)baseUrlBytes.Length - 1,
                        capacity = (ulong)baseUrlBytes.Length - 1
                    };

                    _uuidStr = new RustString
                    {
                        ptr = uuidPtr,
                        len = (ulong)uuidBytes.Length - 1,
                        capacity = (ulong)uuidBytes.Length - 1
                    };

                    ydx_windows_request_init(ref _baseUrlStr, ref _uuidStr);
                    _initialized = true;
                }
                catch
                {
                    Marshal.FreeHGlobal(baseUrlPtr);
                    Marshal.FreeHGlobal(uuidPtr);
                    throw;
                }
            }
            catch (Exception ex)
            {
                System.Diagnostics.Debug.WriteLine($">>> Init failed: {ex.Message}");
                throw;
            }
        }

        private static RustString StringToRustString(string s)
        {
            string clean = PrepareForFfi(s);
            byte[] bytes = Encoding.UTF8.GetBytes(clean + '\0');
            IntPtr ptr = Marshal.AllocHGlobal(bytes.Length);

            try
            {
                Marshal.Copy(bytes, 0, ptr, bytes.Length);

                return new RustString
                {
                    ptr = ptr,
                    len = (ulong)bytes.Length - 1,
                    capacity = (ulong)bytes.Length - 1
                };
            }
            catch
            {
                Marshal.FreeHGlobal(ptr);
                throw;
            }
        }

        private static void FreeRustString(RustString rs)
        {
            if (rs.ptr != IntPtr.Zero)
                Marshal.FreeHGlobal(rs.ptr);
        }

        public static string RequestSignIn(string email, string password)
        {
            RustString emailRs = StringToRustString(email);
            RustString passwordRs = StringToRustString(password);

            try
            {
                VecUint8 result = ydx_windows_request_signin(ref emailRs, ref passwordRs);
                string str = result.ToUtf8String();
                result.Free();
                return str;
            }
            finally
            {
                FreeRustString(emailRs);
                FreeRustString(passwordRs);
            }
        }

        public static string RequestSignUp(string email, string password)
        {
            RustString emailRs = StringToRustString(email);
            RustString passwordRs = StringToRustString(password);

            try
            {
                VecUint8 result = ydx_windows_request_signup(ref emailRs, ref passwordRs);
                string str = result.ToUtf8String();
                result.Free();
                return str;
            }
            finally
            {
                FreeRustString(emailRs);
                FreeRustString(passwordRs);
            }
        }

        public static string RequestSignCheck(string token)
        {
            RustString tokenRs = StringToRustString(token);

            try
            {
                VecUint8 result = ydx_windows_request_signcheck(ref tokenRs);
                string str = result.ToUtf8String();
                result.Free();
                return str;
            }
            finally
            {
                FreeRustString(tokenRs);
            }
        }

        public static string RequestGetUser(string id, string token)
        {
            RustString idRs = StringToRustString(id);
            RustString tokenRs = StringToRustString(token);

            try
            {
                VecUint8 result = ydx_windows_request_get_user(ref idRs, ref tokenRs);
                string str = result.ToUtf8String();
                result.Free();
                return str;
            }
            finally
            {
                FreeRustString(idRs);
                FreeRustString(tokenRs);
            }
        }

        public static string RequestSignOut(string token)
        {
            RustString tokenRs = StringToRustString(token);

            try
            {
                VecUint8 result = ydx_windows_request_signout(ref tokenRs);
                string str = result.ToUtf8String();
                result.Free();
                return str;
            }
            finally
            {
                FreeRustString(tokenRs);
            }
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
            if (ptr == IntPtr.Zero || len == 0 || len > 10 * 1024 * 1024)
                return string.Empty;

            try
            {
                byte[] bytes = new byte[len];
                Marshal.Copy(ptr, bytes, 0, (int)len);
                return Encoding.UTF8.GetString(bytes);
            }
            catch
            {
                return string.Empty;
            }
        }

        public void Free()
        {
            if (ptr != IntPtr.Zero)
            {
                if (len > 0 && len < 10 * 1024 * 1024)
                    NativeMethods.ydx_windows_free_string(ptr, len, cap);

                ptr = IntPtr.Zero;
                len = 0;
                cap = 0;
            }
        }
    }
}