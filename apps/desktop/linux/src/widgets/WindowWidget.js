import GObject from 'gi://GObject';
import Adw from 'gi://Adw';
import Gio from 'gi://Gio';

import './NavigationWidget.js';

export const WindowWidget = GObject.registerClass({
	GTypeName: 'YdxWindowWidget',
	Template: 'resource:///com/keygenqt/ydx/ui/widgets/WindowWidget.ui',
	InternalChildren: [
		'IdNavigationWidget'
	],
}, class extends Adw.ApplicationWindow {

	#settings = new Gio.Settings({
		schema: 'org.gnome.desktop.interface'
	});

	constructor(params) {
		super(params);
		this.#setupColorScheme();
	}

	vfunc_close_request() {
		return super.vfunc_close_request();
	}

	navigation() {
		return this._IdNavigationWidget;
	}

	#setupColorScheme() {
		try {
			const colorScheme = this.#settings.get_string('color-scheme');
			this.#applyColorScheme(colorScheme);
			this.#settings.connect('changed::color-scheme', (settings, key) => {
				const scheme = settings.get_string(key);
				this.#applyColorScheme(scheme);
			});
		} catch (e) {
			console.warn('Could not get color scheme settings:', e);
		}
	}

	#applyColorScheme(scheme) {
		if (scheme === 'prefer-dark') {
			this.add_css_class('dark');
		} else {
			this.remove_css_class('dark');
		}
	}
});
