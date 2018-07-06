import Component from '@ember/component';

export default Component.extend({
    tagName: '',
    willRender() {
        for(let i = 0; i < this.get("data").length; i++) {
            let num = Number(this.get("data")[i].len);
            if(num === 1) {
                this.set('data.' + i + '.word', "Reply");        
            } else {
                this.set('data.' + i + '.word', "Replies");
            }
        }
        
    }
});
