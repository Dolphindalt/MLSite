import Component from '@ember/component';
import $ from 'jquery';
import SHA256 from 'cryptojs/sha256';

export default Component.extend({
    errorMessage: "",
    actions: {
        success() {
            this.get('router').transitionTo('index');
        },
        sign_in() {
            let { username, passwd } = this.getProperties('username', 'passwd');

            var hashword = SHA256(passwd).toString();
            var comp = this;

            $.ajax({
                type: "POST",
                url: "http://127.0.0.1:8000/login",
                dataType: 'json',
                contentType: "application/json; charset=utf-8",
                crossDomain: true,
                data: JSON.stringify({
                    "username":username,
                    "hashword":hashword
                }),
                error: function(xhr, x, y) {
                    console.debug(x);
                    console.debug(y);
                    console.debug(JSON.stringify({
                        "username":username,
                        "hashword":hashword
                    }));
                    if(xhr.status != 200) {
                        comp.set('errorMessage', xhr.responseText);
                    }
                },
                success: function() {
                    console.debug("succ");
                    comp.send('success');
                }
            });
        }
    }
});
