using System.ComponentModel;
using System.Runtime.CompilerServices;

namespace YDX.Base
{
    /// <summary>
    /// Base class for all ViewModels implementing INotifyPropertyChanged.
    /// Provides common functionality for property change notification.
    /// </summary>
    public class ViewModelBase : INotifyPropertyChanged
    {
        /// <summary>
        /// Occurs when a property value changes.
        /// </summary>
        public event PropertyChangedEventHandler? PropertyChanged;

        /// <summary>
        /// Raises the PropertyChanged event.
        /// </summary>
        protected virtual void OnPropertyChanged([CallerMemberName] string? propertyName = null)
        {
            PropertyChanged?.Invoke(this, new PropertyChangedEventArgs(propertyName ?? string.Empty));
        }

        /// <summary>
        /// Sets the property value and raises PropertyChanged if the value changed.
        /// </summary>
        /// <returns>True if value changed, false otherwise.</returns>
        protected bool SetProperty<T>(ref T field, T value, [CallerMemberName] string? propertyName = null)
        {
            if (EqualityComparer<T>.Default.Equals(field, value))
                return false;

            field = value;
            OnPropertyChanged(propertyName);
            return true;
        }

        protected void println(object msg)
        {
            Console.WriteLine(msg);
        }
    }
}