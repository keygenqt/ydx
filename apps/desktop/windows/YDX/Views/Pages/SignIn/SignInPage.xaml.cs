using System.Windows.Controls;

namespace YDX.Views.Pages.SignIn
{
    public partial class SignInPage : Page
    {
        public SignInPage(SignInViewModel viewModel)
        {
            InitializeComponent();
            DataContext = viewModel;
        }
    }
}