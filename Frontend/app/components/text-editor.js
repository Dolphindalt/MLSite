import Component from '@ember/component';

export default Component.extend({
    errorMessage : "",
    ajaxCall: function() {},
    actions: {
        submit_post() {
            ajaxCall();
        }
    }
});
