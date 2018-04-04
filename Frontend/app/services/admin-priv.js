import Service from '@ember/service';
import $ from 'jquery';

export default Service.extend({
    isAdmin() {
        let result;
        $.ajax({
            type: "GET",
            url: "http://127.0.0.1:8000/admin",
            async: false,
        }).always(function(xhr) {
            result = xhr.status;
        });
        return result;
    },
    adminRoute(route) {
        let status = this.isAdmin();
        if(status !== 200) {
            route.transitionTo('error').then(function(nr) {
                nr.send('e403');
            });
        }
    }
});
