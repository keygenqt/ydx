using System.Windows;

namespace YDX.Views
{
    /// <summary>
    /// Main application window with navigation frame
    /// </summary>
    public partial class MainWindow : Window
    {
        public MainWindow()
        {
            InitializeComponent();

            MainFrame.Navigate(new Pages.Welcome.WelcomePage
            {
                DataContext = new ViewModels.Pages.Welcome.WelcomeViewModel()
            });
        }
    }
}