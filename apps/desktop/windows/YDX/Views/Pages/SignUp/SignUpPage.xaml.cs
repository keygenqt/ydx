using Microsoft.UI.Xaml.Controls;

namespace YDX.Views.Pages.SignUp;

public sealed partial class SignUpPage : Page
{
    public SignUpViewModel ViewModel { get; } = App.GetService<SignUpViewModel>();

    public SignUpPage()
    {
        InitializeComponent();
    }
}
