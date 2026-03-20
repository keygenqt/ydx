using CommunityToolkit.Mvvm.ComponentModel;
using CommunityToolkit.Mvvm.Input;
using YDX.Services;

namespace YDX.Views.Windows;

public partial class MainWindowViewModel : ObservableObject
{
    private readonly INavigationService _navigation;
    private readonly ITitleService _titleService;

    [ObservableProperty]
    private bool _canGoBack;

    [ObservableProperty]
    private string _title = "YDX";

    public MainWindowViewModel(
        INavigationService navigation,
        ITitleService titleService)
    {
        _navigation = navigation;
        _titleService = titleService;

        _canGoBack = _navigation.CanGoBack();
        _navigation.CanGoBackChanged += (s, e) => CanGoBack = _navigation.CanGoBack();

        _titleService.TitleChanged += (s, title) => Title = title;
    }

    [RelayCommand]
    private void GoBack() => _navigation.GoBack();
}
