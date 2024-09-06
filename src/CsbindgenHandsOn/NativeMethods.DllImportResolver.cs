using System.Reflection;
using System.Runtime.InteropServices;

namespace CsbindgenHandsOn.Native
{
    internal static unsafe partial class NativeMethods
    {
        static NativeMethods()
        {
            NativeLibrary.SetDllImportResolver(typeof(NativeMethods).Assembly, DllImportResolver);
        }

        static IntPtr DllImportResolver(string libraryName, Assembly assembly, DllImportSearchPath? searchPath)
        {
            if (libraryName == __DllName)
            {
                var path = "runtimes/";
                var extension = "";

                if (RuntimeInformation.IsOSPlatform(OSPlatform.Windows))
                {
                    path += "win-";
                    extension = ".dll";
                }
                else
                {
                    path += "linux-";
                    extension = ".so";
                }

                if (RuntimeInformation.OSArchitecture == Architecture.X64)
                {
                    path += "x64";
                }
                else if (RuntimeInformation.OSArchitecture == Architecture.Arm64)
                {
                    path += "arm64";
                }

                path += $"/native/{(!RuntimeInformation.IsOSPlatform(OSPlatform.Windows)?"lib":"")}{__DllName}{extension}";

                return NativeLibrary.Load(Path.Combine(AppContext.BaseDirectory, path), assembly, searchPath);
            }

            return IntPtr.Zero;
        }
    }
}
