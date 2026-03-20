using System.Threading.Tasks;

namespace YDX.Services;

public interface IDialogService
{
    Task ShowAboutDialogAsync();
    Task<bool> ShowConfirmDialogAsync(string title, string message);
}
