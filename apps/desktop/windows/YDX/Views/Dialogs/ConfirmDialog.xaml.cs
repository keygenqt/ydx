using Microsoft.UI.Xaml.Controls;

namespace YDX.Views.Dialogs
{
    public sealed partial class ConfirmDialog : ContentDialog
    {
        public ConfirmDialog(string title, string message)
        {
            this.InitializeComponent();
            this.Title = title;
            this.MessageText.Text = message;
        }
    }
}
