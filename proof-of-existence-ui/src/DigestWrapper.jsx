import React from 'react';
import { ReactiveComponent } from 'oo7-react';
import { Pretty } from './Pretty';
import { Label } from 'semantic-ui-react';
import { ss58Encode } from 'oo7-substrate';

export class DigestWrapper extends ReactiveComponent {
    constructor() {
        super(["value", "account", "default", "className"])
    }
    render() {
        if (this.ready() || this.props.default == null) {
            if (this.state.value) {
                let time = this.state.value[1];

                // Check if time is 0, which implies not claimed
                if (time.number == 0) {
                    return <Label basic color='green' pointing="left"><span>Unclaimed!</span></Label>
                } else {
                    let owner = ss58Encode(this.state.value[0]);

                    if (ss58Encode(this.state.account) == owner) {
                        return <Label basic color='green' pointing="left"><span>Owner: You!&emsp;|&emsp;When: {time.toLocaleDateString()}</span></Label>
                    } else {
                        return <Label basic color='red' pointing="left"><span>Owner: {owner.substring(0,5) + "..."}&emsp;|&emsp;When: {time.toLocaleDateString()}</span></Label>
                    }
                }
            }
        }

        return <span>{this.props.default}</span>
    }
}
