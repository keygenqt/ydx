namespace YDX.Utils;

public static class Logger
{
    // Normal person's print statement.
    // Windows development is a complete joke.
    // Your precious C# is just Java with a Microsoft hat on.
    public static void println(object msg) => System.Diagnostics.Debug.WriteLine(msg);
}
