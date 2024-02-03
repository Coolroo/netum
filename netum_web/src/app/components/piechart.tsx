'use client'
import dynamic from 'next/dynamic';
const ApexCharts = dynamic(() => import('react-apexcharts'), { ssr: false });

export default function PieChart({props}: {props: {
    options: {},
    series: number[],
    labels: string[]
  }}) {
    return (
        <div className="donut">
          <ApexCharts options={props.options} series={props.series} type="donut" width="380" height="500" />
        </div>
      );
  }