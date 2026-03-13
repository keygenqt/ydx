import GObject from 'gi://GObject';
import Adw from 'gi://Adw';
import Gio from 'gi://Gio';

import './NavigationAppWidget.js';

export const WindowWidget = GObject.registerClass({
	GTypeName: 'YdxWindowWidget',
	Template: 'resource:///com/keygenqt/ydx/ui/widgets/WindowWidget.ui',
	InternalChildren: [
		'IdNavigationAppWidget'
	],
}, class extends Adw.ApplicationWindow {

	#settingsGnome = new Gio.Settings({
		schema: 'org.gnome.desktop.interface'
	});

	constructor(params) {
		super(params);
		this.#checkVersion();
		this.#bindSettings();
		this.#setupColorScheme();
	}

	vfunc_close_request() {
		return super.vfunc_close_request();
	}

	navigation() {
		return this._IdNavigationAppWidget;
	}

	#bindSettings() {
		this.utils.settings.bind(
			this.utils.constants.SettingsKeys.WindowWidth,
			this, 'default-width',
			Gio.SettingsBindFlags.DEFAULT
		);
		this.utils.settings.bind(
			this.utils.constants.SettingsKeys.WindowHeight,
			this, 'default-height',
			Gio.SettingsBindFlags.DEFAULT
		);
	}

	#setupColorScheme() {
		try {
			const colorScheme = this.#settingsGnome.get_string('color-scheme');
			this.#applyColorScheme(colorScheme);
			this.#settingsGnome.connect('changed::color-scheme', (settings, key) => {
				const scheme = settings.get_string(key);
				this.#applyColorScheme(scheme);
			});
		} catch (e) {
			console.warn('Could not get color scheme settings:', e);
		}
	}

	#applyColorScheme(scheme) {
		if (scheme === 'prefer-dark') {
			this.add_css_class('is-dark');
			this.remove_css_class('is-light');
		} else {
			this.add_css_class('is-light');
			this.remove_css_class('is-dark');
		}
	}

    #checkVersion() {
        const savedVersion = this.utils.settings.get(this.utils.constants.SettingsKeys.Version);
        const currentVersion = this.utils.constants.Gtk.dataVersion;
        if (savedVersion !== currentVersion) {
            this.#clearData();
            this.utils.settings.set(this.utils.constants.SettingsKeys.Version, currentVersion);
        }
    }

	#clearData() {
		this.utils.secret.clearAll();
		this.utils.settings.resetAll();
	}
});
