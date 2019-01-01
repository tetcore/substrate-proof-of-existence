import React from 'react';
import {Button, Label} from 'semantic-ui-react';
import {Bond} from 'oo7';
import {ReactiveComponent} from 'oo7-react';
import * as uuid from 'uuid';
import XXH from 'xxhashjs';

export class FileDigestBond extends ReactiveComponent {
	constructor () {
		super(['content', 'disabled']);

		this.changed = this.changed.bind(this)
		this.state = { length: null, digest: null, name: null }
		this.id = uuid.v1()
	}

	changed () {
		const fileButton = document.getElementById(this.id)
		const file = fileButton ? fileButton.files[0] : null

		if (file) {
            this.state.name = file.name;
			var fileReader = new FileReader()
			fileReader.onloadend = e => {
                let fileContents = new Uint8Array(e.target.result)
                let fileDigest = "0x" + XXH.h64( fileContents.buffer, 0 ).toString(16)
                this.props.bond.trigger(fileDigest)
                this.setState({length: fileContents.length, digest: fileDigest})
			}
			fileReader.readAsArrayBuffer(file)
		}
	}

	render () {
		return <span>
			<Button
				content={this.state.content}
				disabled={this.state.disabled}
				as="label"
				htmlFor={this.id}
				label={this.state.digest && this.state.name
					? `${this.state.name} (${this.state.digest})`
					: null
				}
			></Button>
			<input
				hidden
				id={this.id}
				multiple
				type="file"
				onChange={this.changed}
			/>
		</span>
	}
}
