using System.Windows.Input;

namespace YDX.Base
{
    /// <summary>
    /// A command that relays its logic to delegate methods.
    /// Simplifies implementing ICommand by allowing delegates as execute/canExecute.
    /// </summary>
    public class RelayCommand : ICommand
    {
        /// <summary>
        /// The action to execute when the command runs.
        /// </summary>
        private readonly Action _execute;

        /// <summary>
        /// Optional function that determines if the command can execute.
        /// Null means always executable.
        /// </summary>
        private readonly Func<bool>? _canExecute;

        /// <summary>
        /// Initializes a new instance of the RelayCommand class.
        /// </summary>
        /// <param name="execute">The execution logic. Cannot be null.</param>
        /// <param name="canExecute">The execution status logic. Optional, defaults to null (always executable).</param>
        /// <exception cref="ArgumentNullException">Thrown when execute is null.</exception>
        public RelayCommand(Action execute, Func<bool>? canExecute = null)
        {
            _execute = execute ?? throw new ArgumentNullException(nameof(execute));
            _canExecute = canExecute;
        }

        /// <summary>
        /// Determines whether this command can execute in its current state.
        /// </summary>
        /// <param name="parameter">Data used by the command. Not used in this implementation.</param>
        /// <returns>true if this command can be executed; otherwise, false.</returns>
        public bool CanExecute(object? parameter)
        {
            return _canExecute?.Invoke() ?? true;
        }

        /// <summary>
        /// Executes the command logic.
        /// </summary>
        /// <param name="parameter">Data used by the command. Not used in this implementation.</param>
        public void Execute(object? parameter)
        {
            _execute();
        }

        /// <summary>
        /// Occurs when changes occur that affect whether the command should execute.
        /// </summary>
        public event EventHandler? CanExecuteChanged;

        /// <summary>
        /// Raises the CanExecuteChanged event.
        /// Call this method when the canExecute state may have changed.
        /// </summary>
        public void RaiseCanExecuteChanged()
        {
            CanExecuteChanged?.Invoke(this, EventArgs.Empty);
        }
    }
}