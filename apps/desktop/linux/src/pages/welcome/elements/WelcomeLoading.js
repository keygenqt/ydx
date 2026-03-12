import GObject from 'gi://GObject';
import Gtk from 'gi://Gtk';
import GLib from 'gi://GLib';

export const WelcomeLoading = GObject.registerClass({
	GTypeName: 'YdxWelcomeLoading',
	Template: 'resource:///com/keygenqt/ydx/ui/pages/welcome/elements/WelcomeLoading.ui',
	InternalChildren: ['IdProgressBar', 'IdMessage'],
}, class extends Gtk.Box {
	#window
	#messages = [
		_('☕ Brewing coffee'),
		_('🫖 Steeping tea'),
		_('🗄️ Tickling the database'),
		_('🚀 Igniting the reactor'),
		_('🔐 Inspecting tokens'),
		_('🧙 Casting authorization spells'),
		_('🎩 Pulling rabbit out of hat'),
		_('🦄 Feeding unicorns'),
		_('⚡ Accelerating neutrons'),
		_('🧪 Mixing reagents'),
		_('🌱 Watering the server'),
	]
	#currentMessage = 0

	constructor(params) {
		super(params);
		this.#startPulse();
		this.#startMessageRotation();
	}

	vfunc_realize() {
		super.vfunc_realize();
		this.#window = this.get_native();
		this.#currentMessage = Math.floor(Math.random() * this.#messages.length);
		this._IdMessage.set_label(this.#messages[this.#currentMessage]);
	}

	#startPulse() {
		GLib.timeout_add(GLib.PRIORITY_DEFAULT, 50, () => {
			if (this._IdProgressBar) {
				this._IdProgressBar.pulse();
			}
			return GLib.SOURCE_CONTINUE;
		});
	}

	#startMessageRotation() {
		GLib.timeout_add(GLib.PRIORITY_DEFAULT, 3000, () => {
			this.#currentMessage = (this.#currentMessage + 1) % this.#messages.length;
			this._IdMessage.set_label(this.#messages[this.#currentMessage]);
			return GLib.SOURCE_CONTINUE;
		});
	}
});
