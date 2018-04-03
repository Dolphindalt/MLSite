import Service from '@ember/service';
import $ from 'jquery';
import { run } from '@ember/runloop';

export default Service.extend({
    isAdmin() {
        let result = true;
        $.ajax({
            type: "GET",
            url: "http://127.0.0.1:8000/admin",
            async: false,
        }).always(function(xhr) {
            console.debug(xhr.status);
            if(xhr.status !== 200) {
                result = false;
            }
        });
        return result;
    },
    adminRoute(route) {
        console.debug(this.isAdmin());
        if(!this.isAdmin()) {
            console.debug("AAAAAAAAAAAAAAAa");
            route.transitionTo('404');
        }
    }
});
