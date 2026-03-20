using Microsoft.UI.Windowing;
using Windows.Graphics;

namespace YDX.Helpers
{
    internal class WindowHelper
    {
        public static void CenterOnScreen(AppWindow appWindow)
        {
            var displayArea = DisplayArea.GetFromWindowId(
                appWindow.Id,
                DisplayAreaFallback.Primary);

            if (displayArea is not null)
            {
                var centerX = (displayArea.WorkArea.Width - appWindow.Size.Width) / 2;
                var centerY = (displayArea.WorkArea.Height - appWindow.Size.Height) / 2;

                appWindow.Move(new PointInt32((int)centerX, (int)centerY));
            }
        }
    }
}
