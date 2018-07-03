import Service from '@ember/service';

export default Service.extend({
    getDate() {
        var today = new Date();
        var dd = today.getDate();
        var mm = today.getMonth()+1;
        var yyyy = today.getFullYear();

        if(dd < 10) {
            dd = '0'+dd;
        }

        if(mm < 10) {
            mm = '0'+mm;
        }

        today = mm + ' ' + dd + ' ' + yyyy;
        return today.toString();
    },
    getHowMuchTimeAgo(timestamp) {
        var today = new Date();
        var dd = today.getDate();
        var mm = today.getMonth()+1;
        var yyyy = today.getFullYear();

        var long_ago = new Date(timestamp);
        var ldd = long_ago.getDay();
        var lmm = long_ago.getMonth();
        var lyyyy = long_ago.getFullYear();

        if(yyyy > lyyyy) {
            let diff = yyyy - lyyyy;
            return diff + " year" + (diff == 1) ? "" : "s" + " ago";
        } else if(mm > lmm) {
            let diff = mm - lmm;
            return diff + " month" + (diff == 1) ? "" : "s" + " ago";
        } else if(dd > ldd) {
            let diff = dd - ldd;
            return diff + " day" + (diff == 1) ? "" : "s" + " ago";
        } else {
            return "today";
        }
    }
});
