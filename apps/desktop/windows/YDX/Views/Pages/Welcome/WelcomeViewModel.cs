using CommunityToolkit.Mvvm.ComponentModel;
using CommunityToolkit.Mvvm.Input;
using YDX.Services;
using YDX.Views.Pages.SignIn;
using YDX.Views.Pages.SignUp;

namespace YDX.Views.Pages.Welcome;

public partial class WelcomeViewModel(INavigationService _navigation) : ObservableObject
{
    [RelayCommand]
    private void ToSignIn()
    {
        _navigation.NavigateTo<SignInPage>();
    }

    [RelayCommand]
    private void ToSignUp()
    {
        _navigation.NavigateTo<SignUpPage>();
    }
}
