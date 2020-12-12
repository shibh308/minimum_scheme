function update(){
	const str = document.getElementById("input_str").value;
	const target = document.getElementById("output");
	const out_json = window.calc_bs_func(str);

	target.innerHTML = out_json;


	try {
		const json_obj = JSON.parse(out_json);
		const svg_base = document.getElementById("svgBase");
		console.log(json_obj.length);
		console.log(json_obj[0].length);
		const per = 500;
		svg_base.setAttributeNS(null, "viewBox", "0 0 " + per + " " + (json_obj.length * json_obj[0].length * 20 + 20));
		json_obj.forEach((elm, idx) => draw(str, elm, idx));
	} catch (error) {
		console.log(error);
	}
}

function draw(str, obj, idx){
	const n = str.length;
	const str_arr = str.split('');
	const base = document.createElementNS("http://www.w3.org/2000/svg", "svg");
	document.getElementById("svgBase").appendChild(base);

	base.setAttributeNS(null, "x", 10);
	base.setAttributeNS(null, "y", 10 + idx * obj.length * 20);

	const fn = (st, en, y1, y2, col, sw) => {
		const line = document.createElementNS("http://www.w3.org/2000/svg", "line");
		line.setAttributeNS(null, "x1", st);
		line.setAttributeNS(null, "x2", en);
		line.setAttributeNS(null, "y1", y1);
		line.setAttributeNS(null, "y2", y2);
		line.setAttributeNS(null, "stroke", col);
		line.setAttributeNS(null, "stroke-width", sw);
		base.appendChild(line);
	};

	let cnt = 0;
	for (const o of obj) {
		if("pos" in o){
			const st = o.pos * 10 + 10 + 1;
			const en = (o.pos + 1) * 10 + 10 - 1;
			const y = 26 + cnt * 15;
			fn(st, en, y, y, "red", 2);
		}else{
			const sink_st = o.sink * 10 + 10;
			const sink_en = (o.sink + o.len) * 10 + 10;
			const sink_mid = (sink_st + sink_en) / 2
			const source_st = o.source * 10 + 10;
			const source_en = (o.source + o.len) * 10 + 10;
			const source_mid = (source_st + source_en) / 2
			const y_1 = 26 + cnt * 15;
			const y_2 = 30 + cnt * 15;

			fn(sink_st, sink_en, y_1, y_1, "red", 1);
			fn(source_st, source_en, y_2, y_2, "blue", 1);
			fn(sink_mid, source_mid, y_1 + 0.5, y_2 - 0.5, "red", 0.2);
		}
		++cnt;
	}

	str_arr.forEach((c, idx) => {
		const ch = document.createElementNS("http://www.w3.org/2000/svg", "text");
		ch.textContent=c;
		ch.setAttributeNS(null, "x", idx * 10 + 10);
		ch.setAttributeNS(null, "y", 20);
		ch.setAttributeNS(null, "fill", "black");
		ch.setAttributeNS(null, "font-size", 20);
		base.appendChild(ch);
	});
}

const js = import("./node_modules/minimum_scheme/minimum_scheme.js");
js.then(js => {
	window.update = update;
	window.calc_bs_func = js.min_bs;
});
