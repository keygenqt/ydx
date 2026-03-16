using System.Windows.Controls;

namespace YDX.Views.Pages.SignUp
{
    public partial class SignUpPage : Page
    {
        public SignUpPage(SignUpViewModel viewModel)
        {
            InitializeComponent();
            DataContext = viewModel;
        }
    }
}