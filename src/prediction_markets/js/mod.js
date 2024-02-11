import { createChart } from '/lightweight-charts/dist/lightweight-charts.standalone.production.mjs';

export function copy_text_to_clipboard(text) {
    navigator.clipboard.writeText(text)
}

export function create_chart() {
    const chartOptions = {
        layout: { textColor: '#FFF', background: { color: '#111' } },
        grid: { vertLines: { color: 'rgba(255,255,255,.2)' }, horzLines: { color: 'rgba(255,255,255,.2)' } },
        rightPriceScale: { borderColor: 'rgba(255,255,255,.5)' },
        timeScale: { borderColor: 'rgba(197,200,206,.8)', timeVisible: true },
        crosshair: { mode: 0 }
    };
    const chart = createChart(document.getElementById('prediction_markets_chart'), chartOptions);

    const candlestickSeries = chart.addCandlestickSeries({
        priceFormat: { type: 'price', minMove: 1 }
    });
    candlestickSeries.priceScale().applyOptions({
        scaleMargins: { top: .04, bottom: .17 },
        ticksVisible: true,
        borderVisible: false
    })

    const volumeSeries = chart.addHistogramSeries({
        priceFormat: { type: 'volume', precision: 0, },
        color: 'rgba(255,255,255,.6)',
        priceScaleId: 'volume',
        lastValueVisible: false,
        priceLineVisible: false
    });
    volumeSeries.priceScale().applyOptions({
        scaleMargins: { top: .85, bottom: 0 }
    })

    chart.subscribeCrosshairMove(param => {
        let candlestick_span = document.getElementById("prediction_markets_chart_candlestick_series_info")
        let volume_span = document.getElementById("prediction_markets_chart_volume_series_info")

        if (param.time) {
            const candlestickData = param.seriesData.get(candlestickSeries)
            const volumeData = param.seriesData.get(volumeSeries)

            candlestick_span.innerText = `O: ${candlestickData.open}, C: ${candlestickData.close}, H: ${candlestickData.high}, L: ${candlestickData.low}`
            if (volumeData !== undefined) {
                volume_span.innerText = `Volume: ${volumeData.value}`
            } else {
                volume_span.innerText = "Volume: 0"
            }
        } else {
            candlestick_span.innerText = ""
            volume_span.innerText = ""
        }
    })

    return { chart: chart, candlestick_series: candlestickSeries, volume_series: volumeSeries }
}

export function set_chart_data(ctx, data) {
    let candlestick_series_data = [];
    let volume_series_data = [];

    let prev_time = undefined;
    let prev_close = undefined;
    for (let kv of data.Data.candlesticks.entries()) {
        if (prev_time != undefined) {
            let next_timestamp = prev_time + data.Data.interval;
            while (kv[0] != next_timestamp) {
                candlestick_series_data.push(get_candlestick_series_entry_from_close(next_timestamp, prev_close));
                next_timestamp += data.Data.interval;
            }
        }

        candlestick_series_data.push(get_candlestick_series_entry_from_kv(kv[0], kv[1]));
        volume_series_data.push(get_volume_series_entry_from_kv(kv[0], kv[1]));

        prev_time = kv[0];
        prev_close = kv[1].close;
    }

    ctx.candlestick_series.setData(candlestick_series_data);
    ctx.volume_series.setData(volume_series_data);
}

export function update_chart_data(ctx, data) {

    let prev_time = undefined;
    let prev_close = undefined;
    for (let kv of data.Data.candlesticks.entries()) {
        if (prev_time != undefined) {
            let next_timestamp = prev_time + data.Data.interval;
            while (kv[0] != next_timestamp) {
                ctx.candlestick_series.update(get_candlestick_series_entry_from_close(next_timestamp, prev_close));
                next_timestamp += data.Data.interval;
            }
        }

        ctx.candlestick_series.update(get_candlestick_series_entry_from_kv(kv[0], kv[1]));
        ctx.volume_series.update(get_volume_series_entry_from_kv(kv[0], kv[1]));

        prev_time = kv[0];
        prev_close = kv[1].close;
    }
}

function get_candlestick_series_entry_from_kv(timestamp, candlestick) {
    return { time: timestamp, open: candlestick.open, high: candlestick.high, low: candlestick.low, close: candlestick.close }
}

function get_volume_series_entry_from_kv(timestamp, candlestick) {
    return { time: timestamp, value: candlestick.volume }
}

function get_candlestick_series_entry_from_close(timestamp, close) {
    return { time: timestamp, open: close, high: close, low: close, close: close }
}