using System.Windows;

namespace DemoRust
{
    public partial class MainWindow : Window
    {
        public MainWindow()
        {
            InitializeComponent();
        }

        private async void RunTestButton_Click(object sender, RoutedEventArgs e)
        {
            var task = Task.Run(() => NativeMethods.shared_performance_test());
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