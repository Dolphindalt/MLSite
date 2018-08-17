import Service from '@ember/service';

export default Service.extend({
    getDate() {
        var today = new Date();
        var dd = today.getDate();
        var mm = today.getMonth()+1;
        var yyyy = today.getFullYear();

        if(dd < 10) {
            dd = ""+"0"+dd;
        }

        if(mm < 10) {
            mm = ""+"0"+mm;
        }

        today = mm + ' ' + dd + ' ' + yyyy;
        return today.toString();
    },
    getHowMuchTimeAgo(timestamp) {
        var today = new Date();
        var dd = today.getDay();
        var mm = today.getMonth()+1;
        var yyyy = today.getFullYear();

        var long_ago = new Date(timestamp);
        var ldd = long_ago.getDay();
        var lmm = long_ago.getMonth()+1;
        var lyyyy = long_ago.getFullYear();

        if(yyyy > lyyyy) {
            let diff = Math.abs(yyyy - lyyyy);
            let ret = "" + diff + " year";
            if (diff != 1)
                ret += "s"; 
            ret += " ago";
            return ret;
        } else if(mm > lmm) {
            let diff = Math.abs(mm - lmm);
            let ret = "" + diff + " month";
            if (diff != 1)
                ret += "s"; 
            ret += " ago";
            return ret;
        } else if(dd > ldd) {
            let diff = Math.abs(dd - ldd);
            let ret = "" + diff + " day";
            if (diff != 1)
                ret += "s"; 
            ret += " ago";
            return ret;
        } else {
            return "Today";
        }
    }
});
