import Service from '@ember/service';
import $ from 'jquery';

export default Service.extend({
    isAdmin() {
        let result = false;
        $.ajax({
            type: "GET",
            url: "http://127.0.0.1/admin",
            complete: function(xhr) {
                if(xhr.statusCode != 200) {
                    result = true;
                }
            }
        });
        return result == true ? false : true;
    },
    adminRoute(route) {
        if(!this.isAdmin()) {
            route.transitionTo('404');
        }
    }
});
