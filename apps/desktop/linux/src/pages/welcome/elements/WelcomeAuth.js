import GObject from 'gi://GObject';
import Gtk from 'gi://Gtk';
import GLib from 'gi://GLib';

export const WelcomeAuth = GObject.registerClass({
	GTypeName: 'YdxWelcomeAuth',
	Template: 'resource:///com/keygenqt/ydx/ui/pages/welcome/elements/WelcomeAuth.ui',
	InternalChildren: [
		'IdSignIn',
		'IdSignUp',
	],
}, class extends Gtk.Box {
	#window

	vfunc_realize() {
		super.vfunc_realize();
		this.#window = this.get_native();
		this.#actionsConnect();
	}

	#actionsConnect() {
		this._IdSignIn.connect('clicked', () => {
			GLib.timeout_add(GLib.PRIORITY_DEFAULT, 10, () => {
				this.#window.navigation().push(this.utils.constants.Pages.SignIn);
				return GLib.SOURCE_REMOVE;
			});
		});
		this._IdSignUp.connect('clicked', () => {
			this.#window.navigation().push(this.utils.constants.Pages.SignUp);
		});
	}
});
