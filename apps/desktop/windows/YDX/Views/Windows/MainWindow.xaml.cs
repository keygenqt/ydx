using Microsoft.UI;
using Microsoft.UI.Windowing;
using Microsoft.UI.Xaml;
using Windows.Graphics;
using WinRT.Interop;
using YDX.Helpers;
using YDX.Services;
using YDX.Views.Pages.Splash;
using static YDX.Utils.AppConstants;

namespace YDX.Views.Windows;

public sealed partial class MainWindow : Window
{
    public MainWindowViewModel ViewModel { get; } = App.GetService<MainWindowViewModel>();

    public MainWindow()
    {
        InitializeComponent();
        InitializeNavigation();
        InitializeSplash();
        InitializeWindowSize();
        InitializeTitleBar();
        InitializeTheme();
    }

    private void InitializeNavigation()
    {
        var navigation = App.GetService<INavigationService>();
        navigation.Initialize(MainFrame);
    }

    private static void InitializeSplash()
    {
        var navigation = App.GetService<INavigationService>();
        navigation.NavigateTo<SplashPage>();
    }

    private void InitializeWindowSize()
    {
        var hWnd = WindowNative.GetWindowHandle(this);
        var windowId = Win32Interop.GetWindowIdFromWindow(hWnd);
        var appWindow = AppWindow.GetFromWindowId(windowId);

        appWindow.Resize(new SizeInt32(DefaultWindow.Width, DefaultWindow.Height));

        if (appWindow.Presenter is OverlappedPresenter presenter)
        {
            presenter.PreferredMinimumWidth = DefaultWindow.Width;
            presenter.PreferredMinimumHeight = DefaultWindow.Height;
        }

        WindowHelper.CenterOnScreen(appWindow);
    }

    private void InitializeTitleBar()
    {
        ExtendsContentIntoTitleBar = true;
        SetTitleBar(AppTitleBar);
        TitleBarHelper.ApplySystemThemeToCaptionButtons(this);
    }

    private void InitializeTheme()
    {
        var rootElement = (FrameworkElement)Content;
        rootElement.ActualThemeChanged += (s, e) =>
        {
            TitleBarHelper.ApplySystemThemeToCaptionButtons(this);
        };
    }
}
