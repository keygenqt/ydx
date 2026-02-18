using System.Windows;

namespace YDX
{
    public partial class MainWindow : Window
    {
        public MainWindow()
        {
            InitializeComponent();
        }

        private async void RunTestButton_Click(object sender, RoutedEventArgs e)
        {
            var task = Task.Run(() => NativeMethods.ydx_windows_performance_test());
            RunTestButton.Content = "Loading...";
            RunTestButton.IsEnabled = false;
            try
            {
                RunTestButton.Content = $"The test was performed: {await task}ms";
            }
            catch (Exception ex)
            {
                RunTestButton.Content = $"Error: {ex.Message}";
            }
            finally
            {
                RunTestButton.IsEnabled = true;
            }
        }
    }
}