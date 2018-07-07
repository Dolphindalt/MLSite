import Component from '@ember/component';
import { inject as service } from '@ember/service';

export default Component.extend({
    tagName: '',
    date: service('current-date'),
    willRender() {
        if(!this.get('data'))
            return;
        for(let i = 0; i < this.get("data").length; i++) {
            let num = Number(this.get("data")[i].len);
            if(num === 1) {
                this.set('data.' + i + '.word', "Reply");        
            } else {
                this.set('data.' + i + '.word', "Replies");
            }
            this.set('data.' + i + '.timeago', 
                this.get('date').getHowMuchTimeAgo(this.get('data.postdata.' + i + '.datetime')));
        }
        
    }
});
