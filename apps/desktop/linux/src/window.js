/* window.js
 *
 * Copyright 2026 keygenqt
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * 	http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 *
 * SPDX-License-Identifier: Apache-2.0
 */

import GObject from 'gi://GObject';
import Adw from 'gi://Adw';
import GLib from 'gi://GLib';

const Shared = imports.gi.Shared;

export const DemoRustWindow = GObject.registerClass({
    GTypeName: 'DemoRustWindow',
    Template: 'resource:///com/keygenqt/DemoRust/window.ui',
    InternalChildren: ['label', 'IdBtn'],
}, class DemoRustWindow extends Adw.ApplicationWindow {

    constructor(application) {
        super({ application });
        this.#actionsConnect();
    }

    #actionsConnect() {
        this._IdBtn.connect('clicked', () => {
            // Run function
            GLib.timeout_add(GLib.PRIORITY_DEFAULT, 50, () => {
                const value = Shared.performance_test();
                this._IdBtn.label = `The test was performed: ${value}ms`;
                this._IdBtn.sensitive = true;
                return GLib.SOURCE_REMOVE;
            });
            // Start
            this._IdBtn.label = 'Loading...';
            this._IdBtn.sensitive = false;
        });
    }
});
