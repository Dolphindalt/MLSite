import Component from '@ember/component';
import $ from 'jquery';

export default Component.extend({
    classNames: ['home-canvas'],
    tagName: 'canvas',
    width: 600,
    height: 400,
    attributeBindings: ['width', 'height'],
    mycanvas: '',
    triangleData: [ 
        [100, 100, 30, 10],

    ],
    didRender() {
        this.set('mycanvas', Ember.$("canvas")[0]);
        this.send('rain');

        for(let i = 0; i < 30; i++) {
            let data = [
                Math.random() * this.get('width') + 1,
                Math.random() * this.get('height') + 1,
                Math.random() * (50 - 30) + 30,
                Math.random() * 10 + 1,
            ];
            this.get('triangleData').push(data);
        }
    },
    drawTriangle(ctx, x, y, r) {
        ctx.moveTo(x, y + r);
        let v1 = 210 * Math.PI / 180;
        ctx.lineTo((x + r * Math.cos(v1)), y + r * Math.sin(v1));
        let v2 = 330 * Math.PI / 180;
        ctx.lineTo(x + r * Math.cos(v2), y + r * Math.sin(v2));
        ctx.lineTo(x, y + r);
        ctx.stroke();
        ctx.fillStyle = "#000";
        ctx.fill();
    },
    actions: {
        rain() {
            let stuff = () => {
                let ctx = this.get('mycanvas').getContext("2d");
                ctx.clearRect(0, 0, this.get('width'), this.get('height'));
                ctx.beginPath();
                for(let b = this.get('triangleData').length-1; b >= 0; b--) {
                    let data = this.get('triangleData')[b];
                    this.drawTriangle(ctx, data[0], data[1], data[2]);
                    data[0] += data[3];
                    if(data[0] >= this.get('width') + data[2]) data[0] = 0;
                }
            }
            setInterval(stuff, 50);
        }
    }
});
